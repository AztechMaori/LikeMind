import { createSignal } from "solid-js";
import Login from "./Login";
import SignUp from "./SignUp";

export default function LS() {
  const [modal, setModal] = createSignal(true);

  return (
    <>
      <div class=" flex flex-col ">
        {modal() ? (
          <Login setModal={setModal} />
        ) : (
          <SignUp setModal={setModal} />
        )}
      </div>
    </>
  );
}
