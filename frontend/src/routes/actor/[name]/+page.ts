import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const actorId = params.name;
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
            uuid
            name
            actorType
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
    `, { actorId }, fetch);
    
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
