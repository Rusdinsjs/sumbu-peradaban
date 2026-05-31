import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ fetch, params }) => {
  const query = `
    query GetEventDetails($uuid: String!) {
      event(uuid: $uuid) {
        uuid
        title
        description
        iso8601
        islamicDate {
          year
        }
        gregorianDate {
          year
        }
        curationTier
        precision
        sources {
          sourceId
          domain
          title
          author
          referenceText
          reliabilityScore
          subReferences
        }
      }
      fullGraph(limit: 500)
    }
  `;

  try {
    const data = await gql<any>(query, { uuid: params.uuid }, fetch);
    return {
      event: data.event,
      fullGraphData: data.fullGraph || { nodes: [], edges: [] }
    };
  } catch (err) {
    console.error('Failed to fetch event data:', err);
    return {
      event: null,
      fullGraphData: { nodes: [], edges: [] }
    };
  }
};
