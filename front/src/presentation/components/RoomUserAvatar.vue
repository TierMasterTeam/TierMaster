<script setup lang="ts">
import ColorUtils from '@/presentation/utils/colorUtils.ts'
import type RoomUser from '@interfaces/RoomUser.ts'
import { useTierlistRoomStore } from '@stores/tierlistRoomStore.ts'

const props = defineProps<{
  user: RoomUser
  size: 'small' | 'large'
}>()

const tierlistRoomStore = useTierlistRoomStore()

const textColor = ColorUtils.getContrastTextColor(props.user.color)

const getUserInitials = (userName: string) => {
  if (!userName) return ''

  const trimmed = userName.trim()

  const words = trimmed.split(/\s+/)
  if (words.length >= 2) {
    return (words[0][0] + words[1][0]).toUpperCase()
  }

  const upperLetters = [...trimmed].filter((c) => c >= 'A' && c <= 'Z')
  if (upperLetters.length >= 2) {
    return (upperLetters[0] + upperLetters[1]).toUpperCase()
  }

  return trimmed.slice(0, 2).toUpperCase()
}
</script>

<template>
  <div
    :title="user.name"
    :class="[
      'rounded-full',
      {
        'border-2 p-0.5 border-light-green-custom': tierlistRoomStore.roomUser?.id === user.id,
        'w-10 h-10': size === 'large',
        'w-8 h-8': size === 'small',
      },
    ]"
  >
    <div
      class="rounded-full h-full w-full text-sm text-center flex items-center justify-center"
      :style="{
        backgroundColor: user.color,
        color: textColor,
      }"
    >
      <p
        :class="[
          'text-center font-semibold',
          {
            'text-xs': size === 'small',
          },
        ]"
      >
        {{ getUserInitials(user.name) }}
      </p>
    </div>
  </div>
</template>

<style scoped></style>
