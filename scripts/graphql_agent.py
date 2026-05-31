import requests
import json

# ==============================================================================
# Sumbu Peradaban - GraphQL Agent Integration Script
# ==============================================================================
# Skrip ini dibuat untuk digunakan oleh Agen AI (Den) agar dapat memasukkan data 
# secara aman dan valid ke sistem Sumbu Peradaban tanpa mengalami masalah rate-limit 
# browser atau error validasi enum.
# ==============================================================================

GRAPHQL_URL = "http://localhost:8000/graphql" # Sesuaikan dengan URL server backend Anda

class SumbuPeradabanClient:
    def __init__(self, url=GRAPHQL_URL):
        self.url = url
        self.token = None

    def set_token(self, token):
        """Set JWT token secara manual jika sudah punya."""
        self.token = token

    def login(self, email, password):
        """Melakukan login untuk mendapatkan JWT Token."""
        query = """
        mutation Login($input: LoginInput!) {
            login(input: $input) {
                token
            }
        }
        """
        variables = {
            "input": {
                "email": email,
                "password": password
            }
        }
        
        # Kirim tanpa header Authorization untuk login
        response = requests.post(self.url, json={"query": query, "variables": variables})
        data = response.json()
        
        if "errors" in data:
            raise Exception(f"Login gagal: {data['errors']}")
            
        self.token = data["data"]["login"]["token"]
        print("✅ Login berhasil! Token didapatkan.")

    def _execute(self, query, variables):
        """Fungsi helper untuk mengeksekusi GraphQL dengan JWT Header."""
        if not self.token:
            raise Exception("Token tidak tersedia. Lakukan login terlebih dahulu!")
            
        headers = {
            "Content-Type": "application/json",
            "Authorization": f"Bearer {self.token}"
        }
        
        response = requests.post(
            self.url,
            headers=headers,
            json={"query": query, "variables": variables}
        )
        data = response.json()
        
        if "errors" in data:
            raise Exception(f"GraphQL Error: {json.dumps(data['errors'], indent=2)}")
            
        return data["data"]

    # --------------------------------------------------------------------------
    # 1. CREATE ENTITIES
    # --------------------------------------------------------------------------
    def create_actor(self, name, actor_type="INDIVIDUAL", description=""):
        """
        actor_type harus 'INDIVIDUAL' atau 'GROUP'
        """
        query = """
        mutation CreateActor($input: CreateActorInput!) {
            createActor(input: $input) {
                uuid
                name
                actorType
            }
        }
        """
        variables = {
            "input": {
                "name": name,
                "actorType": actor_type,
                "description": description
            }
        }
        result = self._execute(query, variables)
        print(f"👤 Actor dibuat: {result['createActor']['name']} (UUID: {result['createActor']['uuid']})")
        return result["createActor"]["uuid"]

    def create_location(self, name, precision="POINT", lat=None, lng=None):
        """
        precision harus 'POINT', 'AREA', atau 'CONCEPTUAL'
        """
        query = """
        mutation CreateLocation($input: CreateLocationInput!) {
            createLocation(input: $input) {
                uuid
                name
                precision
            }
        }
        """
        variables = {
            "input": {
                "name": name,
                "precision": precision,
                "lat": lat,
                "lng": lng
            }
        }
        result = self._execute(query, variables)
        print(f"🗺️ Location dibuat: {result['createLocation']['name']} (UUID: {result['createLocation']['uuid']})")
        return result["createLocation"]["uuid"]

    def create_event(self, title, description="", year=None):
        """
        precision default disetel ke 'EXACT' (TimePrecision: EXACT, YEAR, DECADE, CENTURY, APPROXIMATE)
        """
        query = """
        mutation CreateEvent($input: CreateEventInput!) {
            createEvent(input: $input) {
                uuid
                title
            }
        }
        """
        variables = {
            "input": {
                "title": title,
                "description": description,
                "precision": "EXACT", 
                # Wajib menyediakan islamic_date dan gregorian_date meskipun tidak lengkap
                "islamicDate": {"year": year or 0},
                "gregorianDate": {"year": year or 0}
            }
        }
        result = self._execute(query, variables)
        print(f"📜 Event dibuat: {result['createEvent']['title']} (UUID: {result['createEvent']['uuid']})")
        return result["createEvent"]["uuid"]

    # --------------------------------------------------------------------------
    # 2. CREATE RELATIONSHIPS
    # --------------------------------------------------------------------------
    def link_actor_to_event(self, actor_uuid, event_uuid, role="Participant"):
        query = """
        mutation LinkActorToEvent($actorUuid: UUID!, $eventUuid: UUID!, $role: String) {
            linkActorToEvent(actorUuid: $actorUuid, eventUuid: $eventUuid, role: $role)
        }
        """
        variables = {
            "actorUuid": actor_uuid,
            "eventUuid": event_uuid,
            "role": role
        }
        self._execute(query, variables)
        print(f"🔗 Linked Actor({actor_uuid}) -> Event({event_uuid}) as {role}")

    def link_event_to_location(self, event_uuid, location_uuid):
        query = """
        mutation LinkEventToLocation($eventUuid: UUID!, $locationUuid: UUID!) {
            linkEventToLocation(eventUuid: $eventUuid, locationUuid: $locationUuid)
        }
        """
        variables = {
            "eventUuid": event_uuid,
            "locationUuid": location_uuid
        }
        self._execute(query, variables)
        print(f"🔗 Linked Event({event_uuid}) -> Location({location_uuid})")

    # --------------------------------------------------------------------------
    # 3. READ ENTITIES (AUDIT)
    # --------------------------------------------------------------------------
    def get_all_actors(self, limit=100):
        query = """
        query GetActors($limit: Int!) {
            actors(limit: $limit) {
                uuid
                name
            }
        }
        """
        result = self._execute(query, {"limit": limit})
        actors = result.get("actors", [])
        print(f"📊 Menemukan {len(actors)} Actor")
        for a in actors:
            print(f"   - {a['name']} ({a['uuid']})")
        return actors

    def get_all_locations(self, limit=100):
        query = """
        query GetLocations($limit: Int!) {
            locations(limit: $limit) {
                uuid
                name
            }
        }
        """
        result = self._execute(query, {"limit": limit})
        locations = result.get("locations", [])
        print(f"📊 Menemukan {len(locations)} Location")
        for loc in locations:
            print(f"   - {loc['name']} ({loc['uuid']})")
        return locations

    def get_all_events(self, limit=100):
        query = """
        query GetEvents($limit: Int!) {
            events(limit: $limit) {
                uuid
                title
            }
        }
        """
        result = self._execute(query, {"limit": limit})
        events = result.get("events", [])
        print(f"📊 Menemukan {len(events)} Event")
        for e in events:
            print(f"   - {e['title']} ({e['uuid']})")
        return events


# ==============================================================================
# CONTOH PENGGUNAAN (SKENARIO OLEH AGEN DEN)
# ==============================================================================
if __name__ == "__main__":
    # Inisialisasi Klien
    client = SumbuPeradabanClient("http://localhost:8000/graphql")
    
    try:
        # 0. Set Token Auth (Atau gunakan client.login("email", "password"))
        # client.login("admin@sjsgroup.site", "password_admin")
        # Atau set manual jika Den mengambilnya dari localStorage browser:
        client.set_token("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...") # GANTI DENGAN TOKEN ASLI
        
        # --- MULAI TRANSAKSI DATA ---
        print("\n--- Memulai Injeksi Data ---\n")

        # 1. Buat Tokoh (Actor)
        actor_id = client.create_actor(
            name="Sultan Agung", 
            actor_type="INDIVIDUAL", 
            description="Raja Kesultanan Mataram"
        )
        
        # 2. Buat Lokasi (Location)
        location_id = client.create_location(
            name="Batavia", 
            precision="AREA"
        )
        
        # 3. Buat Peristiwa (Event)
        event_id = client.create_event(
            title="Penyerangan Batavia", 
            description="Pasukan Mataram menyerang markas VOC di Batavia",
            year=1628
        )
        
        # 4. Buat Relasi (Relationships)
        # Menghubungkan Tokoh ke Peristiwa
        client.link_actor_to_event(actor_id, event_id, role="Commander")
        
        # Menghubungkan Peristiwa ke Lokasi
        client.link_event_to_location(event_id, location_id)

        print("\n✅ Semua data dan relasi berhasil diinjeksi ke Sumbu Peradaban!")

    except Exception as e:
        print(f"\n❌ TERJADI KESALAHAN:\n{e}")
