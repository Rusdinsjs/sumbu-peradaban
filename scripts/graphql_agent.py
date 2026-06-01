import requests
import json
import os
import logging
from datetime import datetime

# ==============================================================================
# Sumbu Peradaban - GraphQL Agent Integration Script
# ==============================================================================
# Skrip ini dibuat untuk digunakan oleh Agen AI (Den) agar dapat memasukkan data 
# secara aman dan valid ke sistem Sumbu Peradaban dengan dukungan UPSERT (Idempotensi).
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
        self._events_cache = []

    def set_token(self, token):
        self.token = token

    def login(self, username, password):
        query = """
        mutation Login($input: LoginInput!) {
            login(input: $input) {
                token
            }
        }
        """
        variables = {"input": {"username": username, "password": password}}
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
        if not self.token: raise Exception("Token tidak tersedia. Lakukan login terlebih dahulu!")
        headers = {"Content-Type": "application/json", "Authorization": f"Bearer {self.token}"}
        try:
            response = requests.post(self.url, headers=headers, json={"query": query, "variables": variables})
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
    # 1. READ ENTITIES (CACHE & AUDIT)
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
        self._events_cache = result.get("events", [])
        return self._events_cache

    def check_entity_exists(self, name, entity_type):
        """Memeriksa apakah entitas dengan nama tersebut sudah ada, mengembalikan UUID jika ada."""
        if not name: return None
        name = name.lower().strip()
        if entity_type == "ACTOR":
            if not self._actors_cache: self.get_all_actors()
            for a in self._actors_cache:
                if a["name"].lower().strip() == name: return a["uuid"]
        elif entity_type == "LOCATION":
            if not self._locations_cache: self.get_all_locations()
            for loc in self._locations_cache:
                if loc["name"].lower().strip() == name: return loc["uuid"]
        elif entity_type == "EVENT":
            if not self._events_cache: self.get_all_events()
            for ev in self._events_cache:
                if ev["title"].lower().strip() == name: return ev["uuid"]
        return None

    # --------------------------------------------------------------------------
    # 2. CREATE ENTITIES
    # --------------------------------------------------------------------------
    def create_actor(self, name, actor_type="INDIVIDUAL", description=""):
        query = """
        mutation CreateActor($input: CreateActorInput!) {
            createActor(input: $input) { uuid name actorType }
        }
        """
        variables = {"input": {"name": name, "actorType": actor_type, "description": description}}
        try:
            result = self._execute(query, variables)
            print(f"👤 Actor dibuat: {result['createActor']['name']} (UUID: {result['createActor']['uuid']})")
            return result["createActor"]["uuid"]
        except Exception:
            print(f"❌ Gagal membuat Actor '{name}'")
            return None

    def create_location(self, name, precision="POINT", lat=None, lng=None):
        query = """
        mutation CreateLocation($input: CreateLocationInput!) {
            createLocation(input: $input) { uuid name precision }
        }
        """
        variables = {"input": {"name": name, "precision": precision, "lat": lat, "lng": lng}}
        try:
            result = self._execute(query, variables)
            print(f"🗺️ Location dibuat: {result['createLocation']['name']} (UUID: {result['createLocation']['uuid']})")
            return result["createLocation"]["uuid"]
        except Exception:
            print(f"❌ Gagal membuat Location '{name}'")
            return None

    def create_event(self, title, description="", year=None):
        query = """
        mutation CreateEvent($input: CreateEventInput!) {
            createEvent(input: $input) { uuid title }
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
        except Exception:
            print(f"❌ Gagal membuat Event '{title}'")
            return None

    # --------------------------------------------------------------------------
    # 3. UPDATE ENTITIES
    # --------------------------------------------------------------------------
    def update_actor(self, uuid, name=None, actor_type=None, description=None):
        query = """
        mutation UpdateActor($uuid: UUID!, $input: UpdateActorInput!) {
            updateActor(uuid: $uuid, input: $input) { uuid name }
        }
        """
        input_data = {}
        if name: input_data["name"] = name
        if actor_type: input_data["actorType"] = actor_type
        if description: input_data["description"] = description
        if not input_data: return uuid # Nothing to update
        
        try:
            self._execute(query, {"uuid": uuid, "input": input_data})
            print(f"🔄 Actor diupdate: {uuid}")
            return uuid
        except Exception:
            print(f"❌ Gagal update Actor {uuid}")
            return None

    def update_location(self, uuid, name=None, precision=None):
        query = """
        mutation UpdateLocation($uuid: UUID!, $input: UpdateLocationInput!) {
            updateLocation(uuid: $uuid, input: $input) { uuid name }
        }
        """
        input_data = {}
        if name: input_data["name"] = name
        if precision: input_data["precision"] = precision
        if not input_data: return uuid
        
        try:
            self._execute(query, {"uuid": uuid, "input": input_data})
            print(f"🔄 Location diupdate: {uuid}")
            return uuid
        except Exception:
            print(f"❌ Gagal update Location {uuid}")
            return None

    def update_event(self, uuid, title=None, description=None):
        query = """
        mutation UpdateEvent($uuid: UUID!, $input: UpdateEventInput!) {
            updateEvent(uuid: $uuid, input: $input) { uuid title }
        }
        """
        input_data = {}
        if title: input_data["title"] = title
        if description: input_data["description"] = description
        if not input_data: return uuid
        
        try:
            self._execute(query, {"uuid": uuid, "input": input_data})
            print(f"🔄 Event diupdate: {uuid}")
            return uuid
        except Exception:
            print(f"❌ Gagal update Event {uuid}")
            return None

    # --------------------------------------------------------------------------
    # 4. UPSERT LOGIC (Idempotency)
    # --------------------------------------------------------------------------
    def upsert_actor(self, name, actor_type="INDIVIDUAL", description=""):
        existing_uuid = self.check_entity_exists(name, "ACTOR")
        if existing_uuid:
            print(f"⚠️ Actor '{name}' sudah ada (UUID: {existing_uuid}). Melakukan update parsial.")
            return self.update_actor(existing_uuid, description=description)
        else:
            uuid = self.create_actor(name, actor_type, description)
            if uuid: self._actors_cache.append({"uuid": uuid, "name": name})
            return uuid

    def upsert_location(self, name, precision="POINT"):
        existing_uuid = self.check_entity_exists(name, "LOCATION")
        if existing_uuid:
            print(f"⚠️ Location '{name}' sudah ada (UUID: {existing_uuid}).")
            return existing_uuid
        else:
            uuid = self.create_location(name, precision)
            if uuid: self._locations_cache.append({"uuid": uuid, "name": name})
            return uuid

    def upsert_event(self, title, description="", year=None):
        existing_uuid = self.check_entity_exists(title, "EVENT")
        if existing_uuid:
            print(f"⚠️ Event '{title}' sudah ada (UUID: {existing_uuid}). Melakukan update parsial.")
            return self.update_event(existing_uuid, description=description)
        else:
            uuid = self.create_event(title, description, year)
            if uuid: self._events_cache.append({"uuid": uuid, "title": title})
            return uuid

    # --------------------------------------------------------------------------
    # 5. CREATE RELATIONSHIPS
    # --------------------------------------------------------------------------
    def link_actor_to_event(self, actor_uuid, event_uuid, role="Participant"):
        if not actor_uuid or not event_uuid: return
        query = """
        mutation LinkActorToEvent($actorUuid: UUID!, $eventUuid: UUID!, $role: String) {
            linkActorToEvent(actorUuid: $actorUuid, eventUuid: $eventUuid, role: $role)
        }
        """
        try:
            self._execute(query, {"actorUuid": actor_uuid, "eventUuid": event_uuid, "role": role})
            print(f"🔗 Linked Actor({actor_uuid}) -> Event({event_uuid}) as {role}")
        except Exception:
            print(f"❌ Gagal melink Actor ke Event")

    def link_event_to_location(self, event_uuid, location_uuid):
        if not location_uuid or not event_uuid: return
        query = """
        mutation LinkEventToLocation($eventUuid: UUID!, $locationUuid: UUID!) {
            linkEventToLocation(eventUuid: $eventUuid, locationUuid: $locationUuid)
        }
        """
        try:
            self._execute(query, {"eventUuid": event_uuid, "locationUuid": location_uuid})
            print(f"🔗 Linked Event({event_uuid}) -> Location({location_uuid})")
        except Exception:
            print(f"❌ Gagal melink Event ke Location")

    def link_event_to_source(self, event_uuid, source_id, sub_references=None):
        if not source_id or not event_uuid: return
        if sub_references is None: sub_references = []
            
        import json
        sub_refs_str = json.dumps(sub_references) if sub_references else ""
            
        query = """
        mutation LinkEventToSource($eventUuid: UUID!, $sourceId: UUID!, $subReferences: String) {
            linkEventToSource(eventUuid: $eventUuid, sourceId: $sourceId, subReferences: $subReferences)
        }
        """
        try:
            self._execute(query, {"eventUuid": event_uuid, "sourceId": source_id, "subReferences": sub_refs_str})
            print(f"🔗 Linked Event({event_uuid}) -> Source({source_id}) [{len(sub_references)} Rincian Rujukan]")
        except Exception:
            print(f"❌ Gagal melink Event ke Source")

    # --------------------------------------------------------------------------
    # 6. QUALITY GATE & BATCH PROCESSING
    # --------------------------------------------------------------------------
    def validate_event(self, event_uuid):
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
            if not event: return ["Event tidak ditemukan"]
            
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

    def process_json_batch(self, filepath):
        if not os.path.exists(filepath):
            print(f"❌ File {filepath} tidak ditemukan.")
            return

        with open(filepath, 'r') as f:
            content = json.load(f)
            
        # Support legacy format (list of events directly)
        if isinstance(content, list):
            batch_type = "EVENTS"
            data = content
        else:
            batch_type = content.get("batch_type", "EVENTS").upper()
            data = content.get("data", [])
            
        print(f"🚀 Memulai Batch Processing (Tahap: {batch_type}) untuk {len(data)} entri...")
        
        if batch_type == "ACTORS":
            for item in data:
                self.upsert_actor(item.get("name"), item.get("type", "INDIVIDUAL"), item.get("description", ""))
        elif batch_type == "LOCATIONS":
            for item in data:
                self.upsert_location(item.get("name"), item.get("precision", "AREA"))
        elif batch_type == "SOURCES":
            # Note: create_source method would be needed for full automation, 
            # but usually sources are managed via Admin UI.
            print("⚠️ Injeksi otomatis untuk SOURCES (Rujukan) disarankan dilakukan melalui Admin Panel untuk menjaga kualitas akademik.")
        elif batch_type == "EVENTS":
            for item in data:
                # 1. Upsert Event
                ev = item.get("event", {})
                event_id = self.upsert_event(ev.get("title"), ev.get("description", ""), ev.get("year"))
                
                if not event_id: continue

                # 2. Upsert Actors
                for actor in item.get("actors", []):
                    act_id = self.upsert_actor(actor.get("name"), actor.get("type", "INDIVIDUAL"))
                    if act_id: self.link_actor_to_event(act_id, event_id, actor.get("role", "Participant"))

                # 3. Upsert Locations
                for loc in item.get("locations", []):
                    loc_id = self.upsert_location(loc.get("name"), loc.get("precision", "AREA"))
                    if loc_id: self.link_event_to_location(event_id, loc_id)

                # 4. Link Sources
                for src in item.get("sources", []):
                    self.link_event_to_source(event_id, src.get("id"), src.get("sub_refs", []))
                
                # 5. Quality Gate
                self.validate_event(event_id)
        else:
            print(f"❌ Tipe batch '{batch_type}' tidak dikenali.")
