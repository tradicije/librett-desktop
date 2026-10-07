<script lang="ts">
  import {
    IconBook2, IconHistory, IconDatabase, IconSettings, IconLayoutSidebarLeftCollapse, IconCash, IconTrophy, IconUser, IconUsers, IconUserPlus, IconArrowLeft, IconArrowRight,
    IconPlus, IconSun, IconMoon, IconDeviceDesktop, IconSearch, IconCircleCheck, IconMinus, IconSquare, IconSquares,
    IconUserMinus, IconArrowBackUp, IconRefresh, IconCircleX, IconPencil, IconTrash, IconHome, IconWorld, IconLayersIntersect, IconList, IconAlertCircle, IconInfoCircle, IconX, IconArrowUp, IconArrowDown,
  } from '@tabler/icons-svelte';

  const icons = {
    guide: IconBook2, history: IconHistory, backup: IconDatabase, settings: IconSettings, minimize: IconMinus, maximize: IconSquare, 'restore-window': IconSquares,
    'arrow-up': IconArrowUp, 'arrow-down': IconArrowDown, info: IconInfoCircle, close: IconX,
    'sidebar-collapse': IconLayoutSidebarLeftCollapse,
    cash: IconCash, trophy: IconTrophy, user: IconUser, users: IconUsers, 'user-plus': IconUserPlus,
    'arrow-left': IconArrowLeft, 'arrow-right': IconArrowRight, plus: IconPlus,
    sun: IconSun, moon: IconMoon, desktop: IconDeviceDesktop, search: IconSearch,
    'check-circle': IconCircleCheck, globe: IconWorld, 'layer-group': IconLayersIntersect,
    withdraw: IconUserMinus, restore: IconArrowBackUp, refresh: IconRefresh, 'absent': IconCircleX, edit: IconPencil, trash: IconTrash, home: IconHome, list: IconList, 'exclamation-circle': IconAlertCircle,
  };
  let { name, size = 20 }: { name: keyof typeof icons; size?: number } = $props();
  const Component = $derived(icons[name]);
</script>

<span class="icon" aria-hidden="true"><Component {size} stroke={1.75} /></span>
