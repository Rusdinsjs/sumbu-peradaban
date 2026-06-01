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
        self._sources_cache = []

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

    def get_all_sources(self, limit=500):
        query = "query GetSources($limit: Int!) { sources(limit: $limit) { sourceId title referenceText domain } }"
        result = self._execute(query, {"limit": limit})
        self._sources_cache = result.get("sources", [])
        return self._sources_cache

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
        elif entity_type == "SOURCE":
            if not self._sources_cache: self.get_all_sources()
            for src in self._sources_cache:
                title = src.get("title")
                ref = src.get("referenceText")
                if title and title.lower().strip() == name: return src["sourceId"]
                if not title and ref and ref.lower().strip() == name: return src["sourceId"]
        return None

    # --------------------------------------------------------------------------
    # 2. CREATE ENTITIES
    # --------------------------------------------------------------------------
    def create_actor(self, input_data):
        query = """
        mutation CreateActor($input: CreateActorInput!) {
            createActor(input: $input) { uuid name actorType }
        }
        """
        try:
            result = self._execute(query, {"input": input_data})
            print(f"👤 Actor dibuat: {result['createActor']['name']} (UUID: {result['createActor']['uuid']})")
            return result["createActor"]["uuid"]
        except Exception as e:
            print(f"❌ Gagal membuat Actor '{input_data.get('name')}': {e}")
            return None

    def create_location(self, input_data):
        query = """
        mutation CreateLocation($input: CreateLocationInput!) {
            createLocation(input: $input) { uuid name precision }
        }
        """
        try:
            result = self._execute(query, {"input": input_data})
            print(f"🗺️ Location dibuat: {result['createLocation']['name']} (UUID: {result['createLocation']['uuid']})")
            return result["createLocation"]["uuid"]
        except Exception as e:
            print(f"❌ Gagal membuat Location '{input_data.get('name')}': {e}")
            return None

    def create_event(self, input_data):
        query = """
        mutation CreateEvent($input: CreateEventInput!) {
            createEvent(input: $input) { uuid title }
        }
        """
        try:
            result = self._execute(query, {"input": input_data})
            print(f"📜 Event dibuat: {result['createEvent']['title']} (UUID: {result['createEvent']['uuid']})")
            return result["createEvent"]["uuid"]
        except Exception as e:
            print(f"❌ Gagal membuat Event '{input_data.get('title')}': {e}")
            return None

    def create_source(self, input_data):
        query = """
        mutation CreateSource($input: CreateSourceInput!) {
            createSource(input: $input) { sourceId title }
        }
        """
        variables = {"input": input_data}
        try:
            result = self._execute(query, variables)
            title = result['createSource'].get('title') or "Unknown"
            print(f"📖 Source dibuat: {title} (UUID: {result['createSource']['sourceId']})")
            return result["createSource"]["sourceId"]
        except Exception as e:
            print(f"❌ Gagal membuat Source: {e}")
            return None

    # --------------------------------------------------------------------------
    # 3. UPDATE ENTITIES
    # --------------------------------------------------------------------------
    def update_actor(self, uuid, input_data):
        if not input_data: return uuid
        query = """
        mutation UpdateActor($uuid: UUID!, $input: UpdateActorInput!) {
            updateActor(uuid: $uuid, input: $input) { uuid name }
        }
        """
        try:
            self._execute(query, {"uuid": uuid, "input": input_data})
            print(f"🔄 Actor diupdate: {uuid}")
            return uuid
        except Exception as e:
            print(f"❌ Gagal update Actor {uuid}: {e}")
            return None

    def update_location(self, uuid, input_data):
        if not input_data: return uuid
        query = """
        mutation UpdateLocation($uuid: UUID!, $input: UpdateLocationInput!) {
            updateLocation(uuid: $uuid, input: $input) { uuid name }
        }
        """
        try:
            self._execute(query, {"uuid": uuid, "input": input_data})
            print(f"🔄 Location diupdate: {uuid}")
            return uuid
        except Exception as e:
            print(f"❌ Gagal update Location {uuid}: {e}")
            return None

    def update_event(self, uuid, input_data):
        if not input_data: return uuid
        query = """
        mutation UpdateEvent($uuid: UUID!, $input: UpdateEventInput!) {
            updateEvent(uuid: $uuid, input: $input) { uuid title }
        }
        """
        try:
            self._execute(query, {"uuid": uuid, "input": input_data})
            print(f"🔄 Event diupdate: {uuid}")
            return uuid
        except Exception as e:
            print(f"❌ Gagal update Event {uuid}: {e}")
            return None

    def update_source(self, uuid, input_data):
        query = """
        mutation UpdateSource($sourceId: UUID!, $input: UpdateSourceInput!) {
            updateSource(sourceId: $sourceId, input: $input) { sourceId title }
        }
        """
        try:
            self._execute(query, {"sourceId": uuid, "input": input_data})
            print(f"🔄 Source diupdate: {uuid}")
            return uuid
        except Exception as e:
            print(f"❌ Gagal update Source {uuid}: {e}")
            return None

    # --------------------------------------------------------------------------
    # 4. UPSERT LOGIC (Idempotency)
    # --------------------------------------------------------------------------
    def upsert_actor(self, item):
        name = item.get("name")
        if not name: return None
        existing_uuid = self.check_entity_exists(name, "ACTOR")
        
        graphql_input = {"name": name}
        if "type" in item: graphql_input["actorType"] = item["type"]
        if "description" in item: graphql_input["description"] = item["description"]
        if "cultural_sphere" in item: graphql_input["culturalSphere"] = item["cultural_sphere"]
        if "birth_year" in item: graphql_input["birthYear"] = item["birth_year"]
        if "death_year" in item: graphql_input["deathYear"] = item["death_year"]
        if "works" in item: graphql_input["works"] = item["works"]
        if "roles" in item: graphql_input["roles"] = item["roles"]
        
        if existing_uuid:
            print(f"⚠️ Actor '{name}' sudah ada (UUID: {existing_uuid}). Melakukan update parsial.")
            return self.update_actor(existing_uuid, graphql_input)
        else:
            if "actorType" not in graphql_input: graphql_input["actorType"] = "INDIVIDUAL"
            uuid = self.create_actor(graphql_input)
            if uuid: self._actors_cache.append({"uuid": uuid, "name": name})
            return uuid

    def upsert_location(self, item):
        name = item.get("name")
        if not name: return None
        existing_uuid = self.check_entity_exists(name, "LOCATION")
        
        graphql_input = {"name": name}
        if "precision" in item: graphql_input["precision"] = item["precision"]
        if "lat" in item: graphql_input["lat"] = float(item["lat"])
        if "lng" in item: graphql_input["lng"] = float(item["lng"])
        if "geography_climate" in item: graphql_input["geographyClimate"] = item["geography_climate"]
        if "historical_role" in item: graphql_input["historicalRole"] = item["historical_role"]
        
        if existing_uuid:
            print(f"⚠️ Location '{name}' sudah ada (UUID: {existing_uuid}).")
            return self.update_location(existing_uuid, graphql_input)
        else:
            if "precision" not in graphql_input: graphql_input["precision"] = "POINT"
            uuid = self.create_location(graphql_input)
            if uuid: self._locations_cache.append({"uuid": uuid, "name": name})
            return uuid

    def upsert_event(self, item):
        title = item.get("title")
        if not title: return None
        existing_uuid = self.check_entity_exists(title, "EVENT")
        
        graphql_input = {"title": title}
        if "description" in item: graphql_input["description"] = item["description"]
        if "precision" in item: graphql_input["precision"] = item["precision"]
        
        islamic_year = item.get("year") or item.get("islamic_year")
        if islamic_year: graphql_input["islamicDate"] = {"year": islamic_year}
        
        gregorian_year = item.get("gregorian_year")
        if gregorian_year: graphql_input["gregorianDate"] = {"year": gregorian_year}
        
        if existing_uuid:
            print(f"⚠️ Event '{title}' sudah ada (UUID: {existing_uuid}). Melakukan update parsial.")
            return self.update_event(existing_uuid, graphql_input)
        else:
            if "precision" not in graphql_input: graphql_input["precision"] = "EXACT"
            if "islamicDate" not in graphql_input: graphql_input["islamicDate"] = {"year": 0}
            if "gregorianDate" not in graphql_input: graphql_input["gregorianDate"] = {"year": 0}
            uuid = self.create_event(graphql_input)
            if uuid: self._events_cache.append({"uuid": uuid, "title": title})
            return uuid

    def upsert_source(self, item):
        title = item.get("title")
        reference_text = item.get("reference_text") or item.get("referenceText")
        check_name = title if title else reference_text
        
        existing_uuid = self.check_entity_exists(check_name, "SOURCE") if check_name else None
        
        graphql_input = {}
        if "domain" in item: graphql_input["domain"] = item["domain"]
        else: graphql_input["domain"] = "General"
        if "title" in item: graphql_input["title"] = item["title"]
        if "author" in item: graphql_input["author"] = item["author"]
        
        pub_era = item.get("publication_era") or item.get("publicationEra")
        if pub_era: graphql_input["publicationEra"] = pub_era
        
        if reference_text: graphql_input["referenceText"] = reference_text
        else: graphql_input["referenceText"] = ""
        
        interp = item.get("interpretation_method") or item.get("interpretationMethod")
        if interp: graphql_input["interpretationMethod"] = interp
        
        score = item.get("reliability_score") or item.get("reliabilityScore")
        if score is not None: graphql_input["reliabilityScore"] = float(score)
        
        assess = item.get("reliability_assessment") or item.get("reliabilityAssessment")
        if assess: graphql_input["reliabilityAssessment"] = assess
        
        if "media_links" in item: graphql_input["mediaLinks"] = item["media_links"]
        elif "mediaLinks" in item: graphql_input["mediaLinks"] = item["mediaLinks"]

        if existing_uuid:
            print(f"⚠️ Source '{check_name}' sudah ada (UUID: {existing_uuid}). Melakukan update parsial.")
            update_input = {k: v for k, v in graphql_input.items() if v}
            return self.update_source(existing_uuid, update_input)
        else:
            uuid = self.create_source(graphql_input)
            if uuid: self._sources_cache.append({"sourceId": uuid, "title": title, "referenceText": reference_text, "domain": graphql_input["domain"]})
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
                self.upsert_actor(item)
        elif batch_type == "LOCATIONS":
            for item in data:
                self.upsert_location(item)
        elif batch_type == "SOURCES":
            for item in data:
                self.upsert_source(item)
        elif batch_type == "EVENTS":
            for item in data:
                # 1. Upsert Event
                ev = item.get("event", {})
                event_id = self.upsert_event(ev)
                
                if not event_id: continue

                # 2. Upsert Actors
                for actor in item.get("actors", []):
                    act_id = self.upsert_actor(actor)
                    if act_id: self.link_actor_to_event(act_id, event_id, actor.get("role", "Participant"))

                # 3. Upsert Locations
                for loc in item.get("locations", []):
                    loc_id = self.upsert_location(loc)
                    if loc_id: self.link_event_to_location(event_id, loc_id)

                # 4. Link Sources
                for src in item.get("sources", []):
                    self.link_event_to_source(event_id, src.get("id"), src.get("sub_refs", []))
                
                # 5. Quality Gate
                self.validate_event(event_id)
        else:
            print(f"❌ Tipe batch '{batch_type}' tidak dikenali.")
