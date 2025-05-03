import { defineStore } from 'pinia'
import { ref, type Ref } from 'vue'
import type { User, LoginCredentials, RegisterCredentials } from '@/domain/interfaces/User'
import api from '@/infra/http/apiClient'

export const useAuthStore = defineStore('authStore', () => {
  const isLoggedIn = ref<boolean | null>(null)

  const user = ref<User | null>(null)

   const checkAuth = async () => {

    // Check if the user is already logged in
    if (isLoggedIn.value === true && user.value) {
      return true
    }
    if (isLoggedIn.value === false) {
      return false
    }
    try {
      const res = await api.get('auth/me') // Adjust the endpoint as needed
      if (res.status === 200) {
        const userData = await res.json() as User
        user.value = userData
        isLoggedIn.value = true
        return true
      }
    } catch (error) {
      isLoggedIn.value = false
      user.value = null
    }
    return false
  }


  const login = async (credentials: LoginCredentials) => {
    try {
      const res = await api.post('auth/login', { json: credentials })
      if (res.status === 200) {
        isLoggedIn.value = true
        return true
      }
    } catch (error) {
      console.error('Login failed:', error)
    }
    return false
  }

  const register = async (credentials: RegisterCredentials) => {
    try {
      const res = await api.post('auth/signup', { json: credentials })
      if (res.status === 200) {
        return true
      }
    } catch (error) {
      console.error('Registration failed:', error)
    }
    return false
  }

  const logout = async () => {
    try {
      const res = await api.post('auth/logout')
      if (res.status === 200) {
        isLoggedIn.value = false
        user.value = null
        return true
      }
    } catch (error) {
      console.error('Logout failed:', error)
    }
    return false
  }

  return { isLoggedIn, user, login, register, checkAuth, logout }
})
