import { defineStore } from 'pinia'
import { io, Socket } from 'socket.io-client'
import { computed, ref } from 'vue'
import type Room from '@interfaces/Room.ts'
import type RoomUser from '@interfaces/RoomUser.ts'
import type { RoomTierList } from '@interfaces/RoomTierlist.ts'

export const useTierlistRoomStore = defineStore('tierlistRoom', () => {
  const socket = ref<Socket | null>(null)
  const isConnected = ref(false)
  const connecting = ref(false)

  const room = ref<Room | null>(null)

  const roomUser = ref<RoomUser | null>(null)
  const tierlist = computed(() => room.value?.tierlist)
  const users = computed(() => room.value?.users ?? [])

  const connect = () => {
    connecting.value = true
    socket.value = io(`${import.meta.env.VITE_API_URL}/ws`, {
      withCredentials: true,
      reconnection: true,
      reconnectionDelayMax: 10000,
    }).connect()

    socket.value?.on('ready', (new_room_user: RoomUser) => {
      isConnected.value = true
      connecting.value = false
      roomUser.value = new_room_user
    })

    socket.value.on('disconnect', () => {
      console.log('Socket disconnected')
      isConnected.value = false
      connecting.value = false
    })

    socket.value.on('join', (new_room: Room) => {
      room.value = new_room
    })

    socket.value.on('leave', (new_room: Room) => {
      room.value = new_room
    })

    socket.value.on('update-tierlist', (new_room: Room) => {
      room.value = new_room
    })
  }

  const disconnect = () => {
    socket.value?.disconnect()
    isConnected.value = false
  }

  const join = async (tierlistId: string) => {
    if (isConnected.value) {
      if (tierlist.value?.id !== tierlistId) {
        leave()
      }

      socket.value?.emit('join', tierlistId)
      return
    }

    if (connecting.value) {
      setTimeout(() => join(tierlistId), 200)
      return
    }

    if (!socket.value) {
      connect()
      setTimeout(() => join(tierlistId), 200)
    }
  }

  const leave = () => {
    socket.value?.emit('leave', roomUser.value)
  }

  const updateTierlist = async (tierlist: RoomTierList) => {
    socket.value?.emit('update-tierlist', tierlist)
  }

  return {
    socket,
    isConnected,
    connecting,

    roomUser,
    room,
    tierlist,
    users,

    join,
    leave,
    updateTierlist,

    disconnect,
  }
})
