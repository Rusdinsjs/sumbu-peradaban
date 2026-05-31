import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ fetch }) => {
  // Try to fetch real events for the timeline/recent activity
  const query = `
    query GetDashboardData {
      events(limit: 500) {
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
      }
      actors(limit: 500) {
        uuid
        name
        actorType
        curationTier
      }
      locations(limit: 500) {
        uuid
        name
        curationTier
      }
      sources(limit: 500) {
        sourceId
      }
    }
  `;

  try {
    const data = await gql<any>(query, {}, fetch);
    return {
      dashboardData: data
    };
  } catch (err) {
    console.error('Failed to fetch dashboard data:', err);
    // Return empty fallback so UI can handle it or show dummy data
    return {
      dashboardData: {
        events: [],
        actors: [],
        locations: [],
        sources: []
      }
    };
  }
};
