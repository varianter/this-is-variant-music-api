using Microsoft.AspNetCore.Mvc;

namespace VariantMusicApi.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class RecommendationController : ControllerBase
    {

        [HttpPost]
        public IActionResult ProcessMusicRecommendation([FromBody] MusicRecommendation recommendation)
        {
            //TODO: process message
            //TODO: fetch metadata from SpotifyApi
            //TODO: Create table row and persist to storage account
            return Ok();
        }
    }
}