import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ fetch }) => {
  const query = `
    query GetMapLocations {
      locations(limit: 50) {
        uuid
        name
        lat
        lng
        curationTier
      }
    }
  `;

  try {
    const data = await gql<any>(query, {}, fetch);
    return {
      locationsData: data.locations || []
    };
  } catch (err) {
    console.error('Failed to fetch map locations:', err);
    return {
      locationsData: []
    };
  }
};
