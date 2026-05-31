import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ fetch }) => {
  const query = `
    query GetTimelineEvents {
      events(limit: 100) {
        uuid
        title
        description
        islamicDate {
          year
        }
        gregorianDate {
          year
        }
        curationTier
      }
    }
  `;

  try {
    const data = await gql<any>(query, {}, fetch);
    return {
      eventsData: data.events || []
    };
  } catch (err) {
    console.error('Failed to fetch timeline events:', err);
    return {
      eventsData: []
    };
  }
};
