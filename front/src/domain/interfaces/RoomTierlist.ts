import type RoomUser from '@interfaces/RoomUser.ts'

export interface RoomTierList {
  id?: string
  name: string
  author: string
  imgCover?: string
  isPublic: boolean
  tags: string[]
  cards: RoomCard[]
  grades: RoomGrade[]
}

export interface RoomGrade {
  name: string
  color: string
  cards: RoomCard[]
}

export interface RoomCard {
  name: string
  image: string
  isDragged: boolean
  draggedBy?: RoomUser
}
