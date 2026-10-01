import '@fontsource-variable/libre-franklin/wght.css';
import { mount } from 'svelte';
import App from './App.svelte';
import './style.css';
import { applyTheme, savedTheme } from './theme';

applyTheme(savedTheme());
mount(App, { target: document.getElementById('app')! });
