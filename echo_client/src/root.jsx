import * as React from "react";
import { createRoot } from "react-dom/client";
import { createTheme, MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";
import App from './app.jsx'

const theme = createTheme({
  /** Put your mantine theme override here */
});

const root = createRoot(document.body);
root.render(
  <>
    <MantineProvider theme={theme}>
      <App/>
      
    </MantineProvider>
  </>
);
