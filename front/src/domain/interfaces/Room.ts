import type RoomUser from '@interfaces/RoomUser.ts'
import type { RoomTierList } from '@interfaces/RoomTierlist.ts'

export default interface Room {
  users: RoomUser[]
  tierlist: RoomTierList
}
