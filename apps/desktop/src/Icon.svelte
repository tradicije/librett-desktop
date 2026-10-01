<script lang="ts">
  import {
    IconTrophy, IconUser, IconUsers, IconUserPlus, IconArrowLeft, IconArrowRight,
    IconPlus, IconSun, IconMoon, IconDeviceDesktop, IconSearch, IconCircleCheck,
    IconWorld, IconLayersIntersect, IconList, IconAlertCircle,
  } from '@tabler/icons-svelte';

  const icons = {
    trophy: IconTrophy, user: IconUser, users: IconUsers, 'user-plus': IconUserPlus,
    'arrow-left': IconArrowLeft, 'arrow-right': IconArrowRight, plus: IconPlus,
    sun: IconSun, moon: IconMoon, desktop: IconDeviceDesktop, search: IconSearch,
    'check-circle': IconCircleCheck, globe: IconWorld, 'layer-group': IconLayersIntersect,
    list: IconList, 'exclamation-circle': IconAlertCircle,
  };
  let { name, size = 20 }: { name: keyof typeof icons; size?: number } = $props();
  const Component = $derived(icons[name]);
</script>

<span class="icon" aria-hidden="true"><Component {size} stroke={1.75} /></span>
