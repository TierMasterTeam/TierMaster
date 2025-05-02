export interface User {
  id: string
  name: string
  email: string
  password: string
  createdAt: Date
  updatedAt: Date
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterCredentials {
  email: string;
  password: string;
  username: string;
}
