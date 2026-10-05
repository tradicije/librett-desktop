import '@fontsource-variable/libre-franklin/wght.css';
import { mount } from 'svelte';
import App from './App.svelte';
import PrintView from './PrintView.svelte';
import './style.css';
import { applyTheme, savedTheme } from './theme';

applyTheme(savedTheme());
mount(new URLSearchParams(window.location.search).has('print') ? PrintView : App, { target: document.getElementById('app')! });
