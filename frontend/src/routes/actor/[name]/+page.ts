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
    // If it's a name, query all actors to find the one matching the name
    try {
      const data = await gql<any>(`
        query GetActorsForMatching {
          actors(limit: 500) {
            uuid
            name
          }
        }
      `, {}, fetch);
      
      const found = (data.actors || []).find((a: any) => 
        a.name.toLowerCase() === nameOrId.toLowerCase() ||
        a.name.toLowerCase().includes(nameOrId.toLowerCase()) ||
        nameOrId.toLowerCase().includes(a.name.toLowerCase())
      );
      if (found) {
        targetUuid = found.uuid;
      }
    } catch (err) {
      console.error('Failed to resolve actor by name:', err);
    }
  }

  if (!targetUuid) {
    return {
      actor: null
    };
  }

  try {
    const data = await gql<any>(`
      query GetActor($actorId: UUID!) {
        actor(uuid: $actorId) {
          uuid
          name
          actorType
          culturalSphere
          birthYear
          deathYear
          curationTier
          works
          roles
          description
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
          relatedActors {
            relationshipType
            actor {
              uuid
              name
              actorType
            }
          }
          visitedLocations {
            uuid
            name
          }
          sources {
            sourceId
            title
            author
            referenceText
            reliabilityScore
          }
        }
      }
    `, { actorId: targetUuid }, fetch);
    
    return {
      actor: data.actor || null
    };
  } catch (err) {
    console.error('Failed to fetch actor detail:', err);
    return {
      actor: null
    };
  }
};
