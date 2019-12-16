#include <iostream>
#include <getopt.h>
#include <osmium/io/any_input.hpp>
#include <osmium/handler.hpp>

struct CountHandler : public osmium::handler::Handler {
    std::uint64_t nodes = 0;
    std::uint64_t ways = 0;
    std::uint64_t relations = 0;

    void node(const osmium::Node& /*node*/) noexcept {
      ++nodes;
    }

    void way(const osmium::Way& /*way*/) noexcept {
      ++ways;
    }

    void relation(const osmium::Relation& /*relation*/) noexcept {
      ++relations;
    }
};

int main(int argc, char **argv) {
  unsigned int zoom;

  static struct option long_options[] = {
    {"help", no_argument, 0, 'h'},
    {"version", no_argument, 0, 'v'},
    {"zoom", required_argument, 0, 'z'},
    {0, 0, 0, 0}
  };

  int c = getopt_long(argc, argv, "hv", long_options, 0);

  while (true) {
    if (c == -1)
        break;

		switch (c) {
  		case 'h':
        std::cout << "phaeton" << std::endl;
        std::cout << "---" << std::endl;
        std::cout << "  use: phaeton [file.osm.pbf] [options]" << std::endl;
        std::exit(0);
      case 'v':
        std::cout << "v1.0.0" << std::endl;
        std::exit(0);
      case 'z':
        zoom = static_cast<unsigned int>(std::atoi(optarg));
        break;
      default:
        std::exit(1);
  	}
  }

  std::cout << "END." << std::endl;

  std::exit(0);
}
