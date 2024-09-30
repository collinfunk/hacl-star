#include <assert.h>
#include <fcntl.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

#include "Hacl_Hash_SHA2.h"

#include "sha2_vectors.h"
#include "test_helpers.h"

typedef Hacl_Hash_SHA2_state_t_256 sha2_state;

int
main()
{
  bool ok = true;

  // Here, I can't really loop over the vectors... because I want to exercise
  // the streaming API with various lengths. Otherwise, in an exemplary test,
  // one would write a for-loop over the test vectors.

  uint8_t tag[32] = {};
  sha2_test_vector* v = vectors;

  sha2_state* s = Hacl_Hash_SHA2_malloc_256();
  assert(Hacl_Hash_SHA2_update_256(s, v->input, 3) == 0);
  Hacl_Hash_SHA2_digest_256(s, tag);
  ok &= compare_and_print(32, tag, v->tag_256);

  v++;
  Hacl_Hash_SHA2_reset_256(s);
  assert(Hacl_Hash_SHA2_update_256(s, NULL, 0) == 0);
  assert(Hacl_Hash_SHA2_update_256(s, v->input, v->input_len) == 0);
  Hacl_Hash_SHA2_digest_256(s, tag);
  ok &= compare_and_print(32, tag, v->tag_256);

  v++;
  Hacl_Hash_SHA2_reset_256(s);
  assert(Hacl_Hash_SHA2_update_256(s, NULL, 0) == 0);
  assert(Hacl_Hash_SHA2_update_256(s, v->input, 16) == 0);
  assert(Hacl_Hash_SHA2_update_256(s, v->input + 16, 16) == 0);
  assert(Hacl_Hash_SHA2_update_256(s, v->input + 32, v->input_len - 32) ==
         0);
  Hacl_Hash_SHA2_digest_256(s, tag);
  ok &= compare_and_print(32, tag, v->tag_256);

  Hacl_Hash_SHA2_free_256(s);

  if (ok)
    return EXIT_SUCCESS;
  else
    return EXIT_FAILURE;
}
