<script lang="ts">
  import {
    IconCash, IconTrophy, IconUser, IconUsers, IconUserPlus, IconArrowLeft, IconArrowRight,
    IconPlus, IconSun, IconMoon, IconDeviceDesktop, IconSearch, IconCircleCheck,
    IconUserMinus, IconArrowBackUp, IconPencil, IconTrash, IconHome, IconWorld, IconLayersIntersect, IconList, IconAlertCircle,
  } from '@tabler/icons-svelte';

  const icons = {
    cash: IconCash, trophy: IconTrophy, user: IconUser, users: IconUsers, 'user-plus': IconUserPlus,
    'arrow-left': IconArrowLeft, 'arrow-right': IconArrowRight, plus: IconPlus,
    sun: IconSun, moon: IconMoon, desktop: IconDeviceDesktop, search: IconSearch,
    'check-circle': IconCircleCheck, globe: IconWorld, 'layer-group': IconLayersIntersect,
    withdraw: IconUserMinus, restore: IconArrowBackUp, edit: IconPencil, trash: IconTrash, home: IconHome, list: IconList, 'exclamation-circle': IconAlertCircle,
  };
  let { name, size = 20 }: { name: keyof typeof icons; size?: number } = $props();
  const Component = $derived(icons[name]);
</script>

<span class="icon" aria-hidden="true"><Component {size} stroke={1.75} /></span>
