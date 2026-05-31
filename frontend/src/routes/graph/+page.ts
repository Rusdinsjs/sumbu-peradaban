import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const ssr = false;

export const load: PageLoad = async ({ fetch }) => {
  try {
    const data = await gql<any>(`
      query {
        fullGraph(limit: 500)
        locations(limit: 100) {
          uuid
          name
          lat
          lng
        }
      }
    `, {}, fetch);
    
    return {
      fullGraphData: data.fullGraph || { nodes: [], edges: [] },
      locationsData: data.locations || []
    };
  } catch (err) {
    console.error('Failed to fetch full graph data:', err);
    return {
      fullGraphData: { nodes: [], edges: [] }
    };
  }
};
