import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ fetch }) => {
  try {
    const data = await gql<any>(`
      query GetSources {
        sources(limit: 100) {
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
        }
      }
    `, {}, fetch);
    
    return {
      sources: data.sources || []
    };
  } catch (err) {
    console.error('Failed to fetch sources:', err);
    return { sources: [] };
  }
};
