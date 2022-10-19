import React, { createContext, useState } from "react";
import { getPathFromSelection } from "../utils";
import { open } from "@tauri-apps/api/dialog";

interface AppData {
  casesFolder: string;
  sourceFile: string;
  timeout: number;
  selectJavaFile: () => void;
  selectTestcaseFolder: () => void;
  setTimeout: (arg: number) => void;
}

export const AppContext = createContext<AppData>({} as AppData);

export const AppContextProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const [sourceFile, setSourceFile] = useState("");
  const [casesFolder, setCasesFolder] = useState("");
  const [timeout, setTimeout] = useState(1);

  async function selectTestcaseFolder() {
    const selected = await open({
      multiple: false,
      directory: true,
    });

    const selectedDir = getPathFromSelection(selected);
    if (selectedDir) {
      setCasesFolder(selectedDir);
    }
  }

  async function selectJavaFile() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Java Source Code",
          extensions: ["java"],
        },
      ],
    });

    const selectedFile = getPathFromSelection(selected);
    if (selectedFile) {
      setSourceFile(selectedFile);
    }
  }

  return (
    <AppContext.Provider
      value={{
        sourceFile,
        casesFolder,
        timeout,
        selectJavaFile,
        selectTestcaseFolder,
        setTimeout,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};
