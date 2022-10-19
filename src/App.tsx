import { Header, Runner, SetupForm } from "./components";
import { AppContextProvider } from "./context/AppContext";

function App() {
  return (
    <AppContextProvider>
      <div className="container mx-auto">
        <div className="my-4 flex flex-col gap-4 items-center ">
          <Header />
          <SetupForm />
          <Runner />
        </div>
      </div>
    </AppContextProvider>
  );
}

export default App;
