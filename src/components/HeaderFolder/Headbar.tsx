

export default function Headbar(){
    
    return(
        <>
        <header class="bg-black text-white p-4">
        <div class="container mx-auto">
            <div class="flex justify-between items-center">
                <a href="#" class="text-2xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300">NOVA</a>
                <nav class="space-x-4">
                    <a href="/" class= "text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300">Home</a>
                    <a href="/trendingpage/" class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300">Trending</a>
                    <a href="/jobspage/" class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300">Jobs</a>
                    <a href="/profilepage/" class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300">Profile</a>
                </nav>
            </div>
        </div>
    </header>
    </>
    )
}