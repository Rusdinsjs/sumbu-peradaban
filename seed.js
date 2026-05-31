const API_URL = 'http://127.0.0.1:8080/graphql';

const locations = [
  { name: 'Makkah Al-Mukarramah', lat: 21.4225, lng: 39.8262, precision: 'POINT', is_transcendental: false, is_global: true, category: 'Kota Suci' },
  { name: 'Madinah Al-Munawwarah', lat: 24.4672, lng: 39.6112, precision: 'POINT', is_transcendental: false, is_global: true, category: 'Pusat Pemerintahan' },
  { name: 'Baitul Maqdis (Al-Quds)', lat: 31.7780, lng: 35.2354, precision: 'POINT', is_transcendental: true, is_global: true, category: 'Tempat Suci' },
  { name: 'Baghdad', lat: 33.3152, lng: 44.3661, precision: 'POINT', is_transcendental: false, is_global: true, category: 'Pusat Keilmuan' },
  { name: 'Cordoba (Qurtuba)', lat: 37.8882, lng: -4.7794, precision: 'POINT', is_transcendental: false, is_global: true, category: 'Peradaban Barat' },
];

const mutation = `
  mutation SeedLocation($input: CreateLocationInput!) {
    createLocation(input: $input) {
      uuid
      name
    }
  }
`;

async function seed() {
  console.log('Seeding locations to Neo4j...');
  for (const loc of locations) {
    try {
      const res = await fetch(API_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          query: mutation,
          variables: {
            input: {
              name: loc.name,
              lat: loc.lat,
              lng: loc.lng,
              precision: loc.precision,
              isTranscendental: loc.is_transcendental,
              isConnectedToGlobal: loc.is_global,
              globalPivotCategory: loc.category
            }
          }
        })
      });
      const data = await res.json();
      if (data.errors) {
        console.error(`Error adding ${loc.name}:`, data.errors);
        process.exit(1);
      } else {
        console.log(`✅ Added: ${loc.name}`);
      }
    } catch (e) {
      console.error(`Failed to reach API for ${loc.name}`, e.message);
      process.exit(1);
    }
  }
  console.log('Done!');
}

seed();
