import "./styles.css";

import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./components/App.tsx";
import { DataProvider } from "@data-client/react";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <DataProvider>
            <App />
        </DataProvider>,
    </React.StrictMode>,
);
