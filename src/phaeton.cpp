#include <iostream>
#include <getopt.h>

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
  std::exit(0);
}
