import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ params, fetch }) => {
  const sourceId = params.id;
  try {
    const data = await gql<any>(`
      query GetSourceAndEvents($sourceId: UUID!) {
        source(sourceId: $sourceId) {
          sourceId
          domain
          title
          author
          publicationEra
          referenceText
          interpretationMethod
          reliabilityScore
          reliabilityAssessment
          mediaLinks {
            mediaType
            url
            title
          }
          actors {
            relationshipType
            actor {
              uuid
              name
              actorType
            }
          }
          locations {
            relationshipType
            location {
              uuid
              name
            }
          }
          events {
            uuid
            title
            description
            gregorianDate {
              year
            }
          }
        }
        events(limit: 1000) {
          uuid
          title
        }
      }
    `, { sourceId }, fetch);
    
    return {
      source: data.source || null,
      allEvents: data.events || []
    };
  } catch (err) {
    console.error('Failed to fetch source:', err);
    return { source: null, allEvents: [] };
  }
};
