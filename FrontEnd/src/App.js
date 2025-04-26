import { useState } from "react";

function App() {
  const [dob, setDob] = useState("");
  const [skinTone, setSkinTone] = useState("fair");
  const [result, setResult] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    
    try {
      const response = await fetch("http://127.0.0.1:3000/get_zodiac", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          dob: dob,
          skin_tone: skinTone,
        }),
      });
      
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      
      const data = await response.json();
      setResult(data);
    } catch (err) {
      console.error("Error:", err);
      setError("Failed to fetch zodiac information. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  // Map zodiac signs to emoji representations
  const getZodiacEmoji = (sign) => {
    const emojiMap = {
      'Aries': '♈',
      'Taurus': '♉',
      'Gemini': '♊',
      'Cancer': '♋',
      'Leo': '♌',
      'Virgo': '♍',
      'Libra': '♎',
      'Scorpio': '♏',
      'Sagittarius': '♐',
      'Capricorn': '♑',
      'Aquarius': '♒',
      'Pisces': '♓'
    };
    return emojiMap[sign] || '✨';
  };

  // Convert color names to actual CSS color values
  const getColorButtons = (colors) => {
    return colors.map((color, index) => {
      const colorMap = {
        'red': 'bg-red-500',
        'blue': 'bg-blue-500',
        'green': 'bg-green-500',
        'yellow': 'bg-yellow-400',
        'purple': 'bg-purple-500',
        'pink': 'bg-pink-500',
        'orange': 'bg-orange-500',
        'teal': 'bg-teal-500',
        'brown': 'bg-amber-700',
        'gold': 'bg-yellow-600',
        'silver': 'bg-gray-300',
        'black': 'bg-black',
        'white': 'bg-white border border-gray-300',
        'gray': 'bg-gray-500',
        'maroon': 'bg-red-800',
        'navy': 'bg-blue-900',
        'olive': 'bg-olive-600',
        'beige': 'bg-amber-100',
        'cream': 'bg-amber-50',
        'coral': 'bg-orange-300',
        'turquoise': 'bg-teal-300',
        'indigo': 'bg-indigo-500',
        'violet': 'bg-violet-500',
        'magenta': 'bg-fuchsia-500',
        'lavender': 'bg-purple-200',
        'cyan': 'bg-cyan-400',
        'emerald': 'bg-emerald-500',
        'ivory': 'bg-amber-50 border border-gray-200',
      };
      
      // Default to a gray if color not found in our map
      const colorClass = color.toLowerCase() in colorMap 
        ? colorMap[color.toLowerCase()] 
        : 'bg-gray-400';
      
      const textClass = ['white', 'cream', 'yellow', 'ivory', 'beige'].includes(color.toLowerCase())
        ? 'text-gray-700' 
        : 'text-white';
      
      return (
        <span 
          key={index} 
          className={`inline-block w-6 h-6 rounded-full mx-1 ${colorClass} ${textClass} text-xs flex items-center justify-center`}
          title={color}
        />
      );
    });
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 to-purple-100 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md mx-auto bg-white rounded-xl shadow-lg overflow-hidden md:max-w-lg">
        <div className="px-6 pt-8 pb-10">
          <div className="text-center mb-8">
            <h1 className="text-3xl font-bold text-indigo-800 mb-2">
              Zodiac Style Advisor
            </h1>
            <p className="text-gray-600">
              Discover your cosmic color palette based on your zodiac sign
            </p>
          </div>

          <form onSubmit={handleSubmit} className="space-y-6">
            <div className="rounded-lg bg-indigo-50 p-4">
              <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center">
                <span className="mr-2 text-indigo-600">📅</span>
                Date of Birth
              </label>
              <input
                type="date"
                value={dob}
                onChange={(e) => setDob(e.target.value)}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                required
              />
            </div>

            <div className="rounded-lg bg-indigo-50 p-4">
              <label className="block text-sm font-medium text-gray-700 mb-2 flex items-center">
                <span className="mr-2 text-indigo-600">👤</span>
                Skin Tone
              </label>
              <select
                value={skinTone}
                onChange={(e) => setSkinTone(e.target.value)}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              >
                <option value="fair">Fair</option>
                <option value="medium">Medium</option>
                <option value="dark">Dark</option>
              </select>
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 relative"
            >
              {loading ? (
                <span className="inline-flex items-center">
                  <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  Processing...
                </span>
              ) : (
                <span className="inline-flex items-center">
                  <span className="mr-2">🎨</span>
                  Get Style Suggestion
                </span>
              )}
            </button>
          </form>

          {error && (
            <div className="mt-6 bg-red-50 border-l-4 border-red-500 p-4 rounded">
              <p className="text-sm text-red-700">{error}</p>
            </div>
          )}

          {result && (
            <div className="mt-8 bg-gradient-to-r from-purple-100 to-indigo-100 p-6 rounded-lg shadow-inner">
              <div className="flex items-center justify-center text-4xl mb-2">
                {getZodiacEmoji(result.zodiac_english)}
              </div>
              
              <h2 className="text-2xl font-bold text-center text-gray-800 mb-1">
                {result.zodiac_english}
              </h2>
              
              <p className="text-center text-lg font-medium text-indigo-600 mb-4">
                {result.zodiac_hindi}
              </p>
              
              <div className="bg-white rounded-md p-4 shadow-sm">
                <h3 className="text-md font-medium text-gray-700 mb-3 flex items-center">
                  <span className="mr-2 text-indigo-500">🎨</span>
                  Your Lucky Colors:
                </h3>
                
                <div className="flex flex-wrap gap-2 mb-3">
                  {getColorButtons(result.lucky_colors)}
                </div>
                
                <p className="text-gray-600 text-sm">
                  {result.lucky_colors.join(", ")}
                </p>
              </div>
              
              <div className="mt-4 text-sm text-center text-gray-500">
                Use these colors in your clothing and accessories for good fortune!
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
