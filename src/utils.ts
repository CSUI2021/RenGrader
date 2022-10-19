export interface IPCMessage {
  error: boolean;
  message: string;
}

export function getPathFromSelection(
  selected: string | string[] | null
): string | null {
  if (selected == null) {
    return null;
  }

  let selectedFile: string;
  if (Array.isArray(selected)) {
    selectedFile = selected[0];
  } else {
    selectedFile = selected;
  }
  return selectedFile;
}
