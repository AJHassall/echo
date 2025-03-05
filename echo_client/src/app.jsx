import * as React from "react";
import { createRoot } from "react-dom/client";
import { createTheme, MantineProvider } from "@mantine/core";
import "@mantine/core/styles.css";

import { Slider } from "./components/vertical_slider.jsx";
import SliderPanel from "./components/SliderPanel/SliderPanel.jsx";
const theme = createTheme({
  /** Put your mantine theme override here */
});

const root = createRoot(document.body);
root.render(
  <>
    <MantineProvider theme={theme}>
      abc
      <SliderPanel edgeSize={30} panelWidth={250}>
        <Slider />
        <Slider />
      </SliderPanel>
      <SliderPanel side="right">
        <div>Right Panel</div>
      </SliderPanel>
    </MantineProvider>
  </>
);
