import requests
import json
import os
import logging
from datetime import datetime

# ==============================================================================
# Sumbu Peradaban - GraphQL Agent Integration Script
# ==============================================================================
# Skrip ini dibuat untuk digunakan oleh Agen AI (Den) agar dapat memasukkan data 
# secara aman dan valid ke sistem Sumbu Peradaban.
# ==============================================================================

GRAPHQL_URL = "http://localhost:8000/graphql"

# ------------------------------------------------------------------------------
# 0. KONFIGURASI LOGGING
# ------------------------------------------------------------------------------
os.makedirs("logs", exist_ok=True)
logging.basicConfig(
    filename="logs/graphql_errors.log",
    level=logging.ERROR,
    format="%(asctime)s - %(levelname)s - %(message)s"
)

class SumbuPeradabanClient:
    def __init__(self, url=GRAPHQL_URL):
        self.url = url
        self.token = None
        # Cache untuk optimasi redundansi cek
        self._actors_cache = []
        self._locations_cache = []

    def set_token(self, token):
        """Set JWT token secara manual jika sudah punya."""
        self.token = token

    def login(self, username, password):
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
                "username": username,
                "password": password
            }
        }
        
        try:
            response = requests.post(self.url, json={"query": query, "variables": variables})
            data = response.json()
            
            if "errors" in data:
                error_msg = f"Login gagal: {data['errors']}"
                logging.error(error_msg)
                raise Exception(error_msg)
                
            self.token = data["data"]["login"]["token"]
            print("✅ Login berhasil! Token didapatkan.")
        except Exception as e:
            logging.error(f"Network/Runtime error during login: {str(e)}")
            raise

    def _execute(self, query, variables):
        """Fungsi helper untuk mengeksekusi GraphQL dengan JWT Header."""
        if not self.token:
            raise Exception("Token tidak tersedia. Lakukan login terlebih dahulu!")
            
        headers = {
            "Content-Type": "application/json",
            "Authorization": f"Bearer {self.token}"
        }
        
        try:
            response = requests.post(
                self.url,
                headers=headers,
                json={"query": query, "variables": variables}
            )
            data = response.json()
            
            if "errors" in data:
                error_msg = f"GraphQL Error: {json.dumps(data['errors'])}"
                logging.error(f"{error_msg} | Variables: {variables}")
                raise Exception(error_msg)
                
            return data.get("data")
        except Exception as e:
            logging.error(f"Execute error: {str(e)} | Query: {query} | Variables: {variables}")
            raise

    # --------------------------------------------------------------------------
    # 1. CREATE ENTITIES (DENGAN ERROR HANDLING)
    # --------------------------------------------------------------------------
    def create_actor(self, name, actor_type="INDIVIDUAL", description=""):
        if self.check_entity_exists(name, "ACTOR"):
            print(f"⚠️ Actor '{name}' sudah ada di database. Melewati pembuatan.")
            return None

        query = """
        mutation CreateActor($input: CreateActorInput!) {
            createActor(input: $input) {
                uuid
                name
                actorType
            }
        }
        """
        variables = {"input": {"name": name, "actorType": actor_type, "description": description}}
        
        try:
            result = self._execute(query, variables)
            print(f"👤 Actor dibuat: {result['createActor']['name']} (UUID: {result['createActor']['uuid']})")
            return result["createActor"]["uuid"]
        except Exception as e:
            print(f"❌ Gagal membuat Actor '{name}': Cek logs/graphql_errors.log")
            return None

    def create_location(self, name, precision="POINT", lat=None, lng=None):
        if self.check_entity_exists(name, "LOCATION"):
            print(f"⚠️ Location '{name}' sudah ada di database. Melewati pembuatan.")
            return None

        query = """
        mutation CreateLocation($input: CreateLocationInput!) {
            createLocation(input: $input) {
                uuid
                name
                precision
            }
        }
        """
        variables = {"input": {"name": name, "precision": precision, "lat": lat, "lng": lng}}
        
        try:
            result = self._execute(query, variables)
            print(f"🗺️ Location dibuat: {result['createLocation']['name']} (UUID: {result['createLocation']['uuid']})")
            return result["createLocation"]["uuid"]
        except Exception as e:
            print(f"❌ Gagal membuat Location '{name}': Cek logs/graphql_errors.log")
            return None

    def create_event(self, title, description="", year=None):
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
                "islamicDate": {"year": year or 0},
                "gregorianDate": {"year": year or 0}
            }
        }
        try:
            result = self._execute(query, variables)
            print(f"📜 Event dibuat: {result['createEvent']['title']} (UUID: {result['createEvent']['uuid']})")
            return result["createEvent"]["uuid"]
        except Exception as e:
            print(f"❌ Gagal membuat Event '{title}': Cek logs/graphql_errors.log")
            return None

    # --------------------------------------------------------------------------
    # 2. CREATE RELATIONSHIPS
    # --------------------------------------------------------------------------
    def link_actor_to_event(self, actor_uuid, event_uuid, role="Participant"):
        if not actor_uuid or not event_uuid: return
        query = """
        mutation LinkActorToEvent($actorUuid: UUID!, $eventUuid: UUID!, $role: String) {
            linkActorToEvent(actorUuid: $actorUuid, eventUuid: $eventUuid, role: $role)
        }
        """
        variables = {"actorUuid": actor_uuid, "eventUuid": event_uuid, "role": role}
        try:
            self._execute(query, variables)
            print(f"🔗 Linked Actor({actor_uuid}) -> Event({event_uuid}) as {role}")
        except Exception as e:
            print(f"❌ Gagal melink Actor ke Event: Cek logs")

    def link_event_to_location(self, event_uuid, location_uuid):
        if not location_uuid or not event_uuid: return
        query = """
        mutation LinkEventToLocation($eventUuid: UUID!, $locationUuid: UUID!) {
            linkEventToLocation(eventUuid: $eventUuid, locationUuid: $locationUuid)
        }
        """
        variables = {"eventUuid": event_uuid, "locationUuid": location_uuid}
        try:
            self._execute(query, variables)
            print(f"🔗 Linked Event({event_uuid}) -> Location({location_uuid})")
        except Exception as e:
            print(f"❌ Gagal melink Event ke Location: Cek logs")

    def link_event_to_source(self, event_uuid, source_id, sub_references=None):
        """
        Melink sebuah Event dengan Source (Rujukan).
        `sub_references` kini menerima list of dicts untuk 3 kolom rujukan.
        Format yang didukung:
        [
            {
                "volumeChapter": "Bab 4 / Surat Al-Baqarah",
                "pageVerse": "Halaman 123 / Ayat 10-15",
                "quoteNotes": "Kutipan dari sumber atau ulasan tafsir"
            }
        ]
        """
        if not source_id or not event_uuid: return
        if sub_references is None:
            sub_references = []
            
        query = """
        mutation LinkEventToSource($eventUuid: UUID!, $sourceId: UUID!, $subReferences: [SubReferenceInput!]) {
            linkEventToSource(eventUuid: $eventUuid, sourceId: $sourceId, subReferences: $subReferences)
        }
        """
        variables = {"eventUuid": event_uuid, "sourceId": source_id, "subReferences": sub_references}
        try:
            self._execute(query, variables)
            print(f"🔗 Linked Event({event_uuid}) -> Source({source_id}) [{len(sub_references)} Rincian Rujukan]")
        except Exception as e:
            print(f"❌ Gagal melink Event ke Source: Cek logs")

    # --------------------------------------------------------------------------
    # 3. READ ENTITIES (AUDIT) & QUALITY GATE
    # --------------------------------------------------------------------------
    def get_all_actors(self, limit=500):
        query = "query GetActors($limit: Int!) { actors(limit: $limit) { uuid name } }"
        result = self._execute(query, {"limit": limit})
        self._actors_cache = result.get("actors", [])
        return self._actors_cache

    def get_all_locations(self, limit=500):
        query = "query GetLocations($limit: Int!) { locations(limit: $limit) { uuid name } }"
        result = self._execute(query, {"limit": limit})
        self._locations_cache = result.get("locations", [])
        return self._locations_cache

    def get_all_events(self, limit=500):
        query = "query GetEvents($limit: Int!) { events(limit: $limit) { uuid title } }"
        result = self._execute(query, {"limit": limit})
        return result.get("events", [])

    def check_entity_exists(self, name, entity_type):
        """Memeriksa apakah entitas dengan nama tersebut sudah ada untuk menghindari duplikasi."""
        name = name.lower().strip()
        if entity_type == "ACTOR":
            if not self._actors_cache: self.get_all_actors()
            return any(a["name"].lower().strip() == name for a in self._actors_cache)
        elif entity_type == "LOCATION":
            if not self._locations_cache: self.get_all_locations()
            return any(loc["name"].lower().strip() == name for loc in self._locations_cache)
        return False

    def validate_event(self, event_uuid):
        """Quality Gate: Memastikan sebuah event memiliki relasi yang lengkap."""
        query = """
        query ValidateEvent($uuid: UUID!) {
            event(uuid: $uuid) {
                title
                actors { uuid }
                locations { uuid }
                sources { sourceId }
            }
        }
        """
        try:
            result = self._execute(query, {"uuid": event_uuid})
            event = result.get("event")
            if not event:
                return ["Event tidak ditemukan"]
            
            missing = []
            if not event.get("actors"): missing.append("Actor (Tokoh)")
            if not event.get("locations"): missing.append("Location (Lokasi)")
            if not event.get("sources"): missing.append("Source (Rujukan)")
            
            if missing:
                print(f"⚠️ QUALITY GATE FAILED for '{event['title']}': Missing {', '.join(missing)}")
            else:
                print(f"✅ QUALITY GATE PASSED for '{event['title']}': Data lengkap!")
            return missing
        except Exception as e:
            logging.error(f"Failed to validate event {event_uuid}: {str(e)}")
            return ["Error during validation"]

    # --------------------------------------------------------------------------
    # 4. STANDARDISASI BATCH INPUT (CONTOH)
    # --------------------------------------------------------------------------
    def process_json_batch(self, filepath):
        """
        Membaca data masal dari JSON dan memprosesnya secara terstruktur.
        Ekspektasi struktur JSON:
        [
            {
                "event": {"title": "Perang Badar", "year": 624},
                "actors": [{"name": "Rasulullah SAW", "type": "INDIVIDUAL", "role": "Commander"}],
                "locations": [{"name": "Lembah Badar", "precision": "AREA"}],
                "sources": [
                    {
                        "id": "UUID-SOURCE-DI-DB", 
                        "sub_refs": [
                            {
                                "volumeChapter": "Jilid 1 / Bab 2",
                                "pageVerse": "Halaman 120",
                                "quoteNotes": "Penjelasan tentang logistik perang"
                            }
                        ]
                    }
                ]
            }
        ]
        """
        if not os.path.exists(filepath):
            print(f"❌ File {filepath} tidak ditemukan.")
            return

        with open(filepath, 'r') as f:
            data = json.load(f)
            
        print(f"🚀 Memulai Batch Processing untuk {len(data)} entri...")
        for item in data:
            # 1. Create Event
            ev = item.get("event", {})
            event_id = self.create_event(ev.get("title"), ev.get("description", ""), ev.get("year"))
            
            if not event_id: continue

            # 2. Actors
            for actor in item.get("actors", []):
                act_id = self.create_actor(actor.get("name"), actor.get("type", "INDIVIDUAL"))
                if act_id: self.link_actor_to_event(act_id, event_id, actor.get("role", "Participant"))

            # 3. Locations
            for loc in item.get("locations", []):
                loc_id = self.create_location(loc.get("name"), loc.get("precision", "AREA"))
                if loc_id: self.link_event_to_location(event_id, loc_id)

            # 4. Sources
            for src in item.get("sources", []):
                self.link_event_to_source(event_id, src.get("id"), src.get("sub_refs", []))
            
            # 5. Quality Gate
            self.validate_event(event_id)

# ==============================================================================
# CONTOH PENGGUNAAN OLEH AGEN DEN
# ==============================================================================
if __name__ == "__main__":
    client = SumbuPeradabanClient()
    # client.login("admin", "password_admin")
    
    # client.process_json_batch("scripts/data_input/batch_01.json")
