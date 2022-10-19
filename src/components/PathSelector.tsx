interface PathSelectorProps {
  label: string;
  path: string;
  onClick: () => void;
}

export const PathSelector = ({ label, path, onClick }: PathSelectorProps) => {
  return (
    <div className="flex flex-row gap-2 items-center">
      <label className="basis-1/5">{label}</label>
      <p className="basis-3/5">{path || "None selected"}</p>
      <button onClick={onClick} className="btn btn-primary basis-1/5">
        Select
      </button>
    </div>
  );
};
