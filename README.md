# SQLite Visual Representation

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Install daisyUI and official Tailwind CSS Typography plugin:


```bash
npm i -D daisyui@latest @tailwindcss/typography
```

4. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

5. In the root of the project create database examples:
```bash
make setup
```

6. Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```
7. Open the browser to http://localhost:8080