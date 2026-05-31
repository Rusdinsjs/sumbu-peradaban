import type { PageLoad } from './$types';
import { gql } from '$lib/graphql/client';

export const load: PageLoad = async ({ fetch }) => {
  try {
    const data = await gql<any>(`
      query GetActors {
        actors(limit: 200) {
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
        }
      }
    `, {}, fetch);
    
    return {
      actors: data.actors || []
    };
  } catch (err) {
    console.error('Failed to fetch actors list:', err);
    return {
      actors: []
    };
  }
};
