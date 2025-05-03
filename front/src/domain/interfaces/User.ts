export interface User {
  id: string
  email: string
  username: string
  createdAt?: Date
  updatedAt?: Date
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterCredentials {
  email: string;
  username: string;
  password: string;
}