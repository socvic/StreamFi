export default function Home() {
  return (
    <main className="min-h-screen p-8">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-4xl font-bold mb-8">StreamFi</h1>
        <p className="text-xl mb-8 text-gray-400">
          Tokenized Future Income Protocol on Stellar
        </p>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div className="p-6 border rounded-lg">
            <h2 className="text-xl font-semibold mb-2">Income Tokens</h2>
            <p className="text-gray-400">Create tokens backed by future income</p>
          </div>
          <div className="p-6 border rounded-lg">
            <h2 className="text-xl font-semibold mb-2">Streaming</h2>
            <p className="text-gray-400">Continuous payment distribution</p>
          </div>
          <div className="p-6 border rounded-lg">
            <h2 className="text-xl font-semibold mb-2">Reputation</h2>
            <p className="text-gray-400">Track payment reliability</p>
          </div>
          <div className="p-6 border rounded-lg">
            <h2 className="text-xl font-semibold mb-2">Marketplace</h2>
            <p className="text-gray-400">Trade income positions</p>
          </div>
        </div>
      </div>
    </main>
  )
}
