import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const nameOrId = decodeURIComponent(params.name);
  
  // Check if nameOrId is a UUID
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
  let targetUuid: string | null = null;
  
  if (uuidRegex.test(nameOrId)) {
    targetUuid = nameOrId;
  } else {
    // If it's a name, query all locations to find the one matching the name
    try {
      const data = await gql<any>(`
        query GetLocationsForMatching {
          locations(limit: 500) {
            uuid
            name
          }
        }
      `, {}, fetch);
      
      const found = (data.locations || []).find((l: any) => 
        l.name.toLowerCase() === nameOrId.toLowerCase() ||
        l.name.toLowerCase().includes(nameOrId.toLowerCase()) ||
        nameOrId.toLowerCase().includes(l.name.toLowerCase())
      );
      if (found) {
        targetUuid = found.uuid;
      }
    } catch (err) {
      console.error('Failed to resolve location by name:', err);
    }
  }

  if (!targetUuid) {
    return {
      location: null
    };
  }

  try {
    const data = await gql<any>(`
      query GetLocationDetail($uuid: UUID!) {
        location(uuid: $uuid) {
          uuid
          name
          locationType
          region
          lat
          lng
          precision
          isTranscendental
          curationTier
          geographyClimate
          demographics
          socioCultural
          historicalRole
          mediaLinks {
            mediaType
            url
            title
          }
          timeline {
            uuid
            title
            description
            gregorianDate {
              year
            }
          }
          relatedLocations {
            uuid
            name
            lat
            lng
          }
          actors {
            uuid
            name
            actorType
          }
          sources {
            sourceId
            title
            author
            reliabilityScore
          }
        }
      }
    `, { uuid: targetUuid }, fetch);
    
    return {
      location: data.location || null
    };
  } catch (err) {
    console.error('Failed to fetch location details:', err);
    return {
      location: null
    };
  }
};
