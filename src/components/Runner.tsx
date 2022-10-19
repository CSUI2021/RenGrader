import React, { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { AppContext } from "../context/AppContext";
import { IPCMessage } from "../utils";
import { Modal } from "./Modal";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

export const Runner = () => {
  const appData = useContext(AppContext);
  const [debug, setDebug] = useState(false);
  const [isLoading, setLoading] = useState(false);

  const [debugMsg, setDebugMsg] = useState("");

  const [result, setResult] = useState<string[]>([]);
  const [show, setShow] = useState(false);
  const [err, setErr] = useState("");

  useEffect(() => {
    let unlistenFunc: UnlistenFn;

    (async () => {
      const unlisten = await listen<IPCMessage>("onDebugMessage", (event) => {
        setDebugMsg((curr) =>
          (curr + "\n" + event.payload.message.trimEnd()).trimStart()
        );
      });
      unlistenFunc = unlisten;
    })();

    return () => unlistenFunc?.();
  }, []);

  const showError = (message: string) => {
    setErr(message);
    setShow(true);
  };

  return (
    <>
      <div className="flex flex-row items-center gap-2">
        <button
          onClick={async () => {
            setDebugMsg("");
            setResult([]);
            setErr("");
            setLoading(true);

            if (appData.sourceFile == "" || appData.casesFolder == "") {
              showError("Please set source code file and test cases folder!");
              setLoading(false);
              return;
            }

            const result = await invoke<IPCMessage>("run_tests", {
              sourcePath: appData.sourceFile,
              testCasesPath: appData.casesFolder,
              timeout: appData.timeout * 1000,
            });
            setLoading(false);

            if (result.error) {
              showError(result.message.trim());
              return;
            }

            setResult(result.message.split(",").slice(0, -1));
          }}
          className="btn btn-primary"
        >
          Run
        </button>
        <button onClick={() => setDebug(!debug)} className="btn">
          {debug ? "Hide" : "Show"} Debug
        </button>
      </div>

      {debug && (
        <textarea
          className="font-mono textarea w-full textarea-bordered resize-none min-h-[12rem]"
          value={debugMsg}
          readOnly
        />
      )}

      {isLoading && (
        /* @ts-ignore */
        <i className="radial-progress animate-spin" style={{ "--value": 80 }} />
      )}

      <div className="grid grid-cols-4 gap-2 w-4/5">
        {result &&
          result.map((val, i) => (
            <p
              key={i}
              className={"border p-2 " + (val != "AC" && "border-error")}
            >
              TC{i}: {val}
            </p>
          ))}
      </div>

      <Modal show={show} title="Error" onClose={() => setShow(false)}>
        <pre>{err}</pre>
      </Modal>
    </>
  );
};
