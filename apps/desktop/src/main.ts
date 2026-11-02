import '@fontsource-variable/libre-franklin/wght.css';
import { mount } from 'svelte';
import App from './App.svelte';
import PrintView from '$lib/features/reports/PrintView.svelte';
import './style.css';
import { applyTheme, savedTheme } from '$lib/theme';

applyTheme(savedTheme());
mount(new URLSearchParams(window.location.search).has('print') ? PrintView : App, { target: document.getElementById('app')! });
