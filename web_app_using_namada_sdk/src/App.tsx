import { useEffect, useState } from "react";
import "./App.css";
import { init, performRequestFromUi } from "./utils/namadaSdk/index";
function App() {
  const [inputFieldContent, setInputFieldContent] = useState(
    "atest1d9khqw36xyuyxdf5g4qnjwfkxvc52vf48qu5yseexcm52wpcg56ygd3sxdp52s2z89znsvz93amfph"
  );
  useEffect(() => {
    // we have to put this to async as we call async funcs
    const asyncBlock = async () => {
      await init();
    };
    asyncBlock();
  }, []);

  const fetchBalanceOfAddress = () => {
    // we have to put this to async as we call async funcs
    const asyncBlock = async () => {
      await performRequestFromUi();
    };
    asyncBlock();
  };

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ display: "flex", flexDirection: "column", width: "50%" }}>
          <input
            value={inputFieldContent}
            onChange={(event) => {
              setInputFieldContent(event.target.value);
            }}
            style={{ height: "32px", margin: "0 0 16px" }}
          />
          <button
            onClick={() => {
              fetchBalanceOfAddress();
            }}
            style={{ height: "32px", margin: "0 0 16px" }}
          >
            query balance of address
          </button>
        </div>
      </header>
    </div>
  );
}

export default App;
