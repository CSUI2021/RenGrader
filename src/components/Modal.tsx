import React from "react";

interface ModalProps {
  show: boolean;
  title: string;
  children: React.ReactNode;
  onClose: () => void;
}

export const Modal = ({ title, children, show, onClose }: ModalProps) => {
  return (
    <>
      <input
        type="checkbox"
        id="my-modal"
        checked={show}
        className="modal-toggle"
        readOnly
      />

      <div className="modal">
        <div className="modal-box">
          <h3 className="font-bold text-lg">{title}</h3>
          <div className="py-4">{children}</div>
          <div className="modal-action">
            <label htmlFor="my-modal" className="btn" onClick={onClose}>
              Close
            </label>
          </div>
        </div>
      </div>
    </>
  );
};
