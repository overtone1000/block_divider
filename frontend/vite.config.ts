import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type UserConfigExport } from 'vite';

export default defineConfig(
	({ command, mode, ssrBuild }) => {
		const retval: UserConfigExport = {
			plugins: [sveltekit()],
			server: {
				open: "/admin", //automatically open the admin panel on server start
			},
		};

		if (command === "serve") {
			process.env.VITE_POST_ROOT = "http://localhost:8181/";
		}
		else if (command === "build") {
			process.env.VITE_POST_ROOT = "";
		}

		return retval;
	}
);