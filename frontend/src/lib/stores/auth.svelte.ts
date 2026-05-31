import { browser } from '$app/environment';

export interface User {
  id: string;
  username: string;
  role: 'VISITOR' | 'CONTRIBUTOR' | 'EDITOR' | 'REVIEWER' | 'ADMIN';
}

class AuthStore {
  user = $state<User | null>(null);
  token = $state<string | null>(null);

  constructor() {
    if (browser) {
      const storedToken = localStorage.getItem('sumbu_token');
      const storedUser = localStorage.getItem('sumbu_user');
      
      if (storedToken && storedUser) {
        this.token = storedToken;
        try {
          this.user = JSON.parse(storedUser);
        } catch(e) {
          console.error("Failed to parse user from local storage", e);
        }
      }
    }
  }

  login(token: string, user: User) {
    this.token = token;
    this.user = user;
    if (browser) {
      localStorage.setItem('sumbu_token', token);
      localStorage.setItem('sumbu_user', JSON.stringify(user));
    }
  }

  logout() {
    this.token = null;
    this.user = null;
    if (browser) {
      localStorage.removeItem('sumbu_token');
      localStorage.removeItem('sumbu_user');
    }
  }

  get isAuthenticated() {
    return this.token !== null;
  }

  get isAdmin() {
    return this.user?.role === 'ADMIN';
  }

  get isEditor() {
    return (
      this.user?.role === 'CONTRIBUTOR' ||
      this.user?.role === 'EDITOR' ||
      this.user?.role === 'REVIEWER' ||
      this.user?.role === 'ADMIN'
    );
  }

  get isReviewer() {
    return this.user?.role === 'REVIEWER' || this.user?.role === 'ADMIN';
  }
}

export const auth = new AuthStore();
