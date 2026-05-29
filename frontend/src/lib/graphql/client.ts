const API_URL = typeof window !== 'undefined'
  ? (import.meta.env.VITE_API_URL || 'http://localhost:8080')
  : 'http://backend:8080';

export interface GraphQLResponse<T> {
  data?: T;
  errors?: Array<{ message: string; locations?: Array<{ line: number; column: number }> }>;
}

export async function gql<T = unknown>(
  query: string,
  variables?: Record<string, unknown>
): Promise<T> {
  const response = await fetch(`${API_URL}/graphql`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query, variables }),
  });

  if (!response.ok) {
    throw new Error(`GraphQL request failed: ${response.status} ${response.statusText}`);
  }

  const result: GraphQLResponse<T> = await response.json();

  if (result.errors && result.errors.length > 0) {
    throw new Error(result.errors.map(e => e.message).join('; '));
  }

  if (!result.data) {
    throw new Error('No data returned from GraphQL');
  }

  return result.data;
}
