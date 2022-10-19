import React, { useContext } from "react";
import { PathSelector } from ".";
import { AppContext } from "../context/AppContext";

export const SetupForm = () => {
  const {
    timeout,
    sourceFile,
    selectJavaFile,
    casesFolder,
    selectTestcaseFolder,
    setTimeout,
  } = useContext(AppContext);

  return (
    <div className="flex flex-col gap-2 min-w-[80vw]">
      <PathSelector
        label="Source Code"
        path={sourceFile}
        onClick={selectJavaFile}
      />
      <PathSelector
        label="Test Cases Folder"
        path={casesFolder}
        onClick={selectTestcaseFolder}
      />
      <div className="flex flex-row gap-2 items-center">
        <label className="basis-1/5">Time Limit</label>
        <input
          className="basis-4/5 input input-bordered"
          type="number"
          onChange={(e) => setTimeout(Number(e.target.value))}
          min={1}
          value={timeout}
        />
      </div>
    </div>
  );
};
