import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [filePath, setFilePath] = useState("");
  const [fileName, setFileName] = useState("");
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState("");

  const selectFile = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "PDF",
          extensions: ["pdf"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      setFilePath(selected);

      const parts = selected.split("/");
      setFileName(parts[parts.length - 1]);
    }
  };

  const compress = async () => {
    if (!filePath) return;

    setLoading(true);
    setResult("");

    try {
      const output = await invoke("compress_pdf", {
        inputPath: filePath,
      });

      setResult(String(output));
    } catch (err) {
      console.error(err);
      setResult("Compression failed");
    }

    setLoading(false);
  };

  return (
    <div className="min-h-screen bg-zinc-950 text-white flex items-center justify-center p-8">
      <div className="w-full max-w-2xl">
        <div className="border-2 border-dashed border-zinc-700 rounded-3xl p-20 text-center">
          <h1 className="text-5xl font-bold mb-6">
            PDF Compressor
          </h1>

          <p className="text-zinc-400 mb-8 text-lg">
            Select your PDF file
          </p>

          <button
            onClick={selectFile}
            className="bg-white text-black px-6 py-3 rounded-2xl font-semibold hover:opacity-80 transition"
          >
            Select PDF
          </button>

          <div className="mt-6 text-zinc-400">
            {fileName || "No file selected"}
          </div>

          <button
            onClick={compress}
            disabled={!filePath || loading}
            className="mt-8 bg-green-500 text-black px-6 py-3 rounded-2xl font-bold disabled:opacity-50"
          >
            {loading ? "Compressing..." : "Compress PDF"}
          </button>

          {result && (
            <div className="mt-8 text-green-400 break-all">
              {result}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
