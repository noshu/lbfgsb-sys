AFLAG = crus
SFlAG = -shared -o
FFLAGS = -O3 -Wall -fexceptions -g -Wno-uninitialized -c -fPIC -o
LBFGSB  = lbfgsb.f
LINPACK = linpack.f
BLAS    = blas.f
TIMER   = timer.f
STRING = string.f
static :
	@echo "static linking selected"
	@echo "Compiling"
	$(FC) $(LBFGSB) $(FFLAGS) $(OUTPUT)/lbfgsb.o
	$(FC) $(LINPACK) $(FFLAGS) $(OUTPUT)/linpack.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	$(FC) $(STRING) $(FFLAGS) $(OUTPUT)/string.o
	@echo "creating static library"
	$(AR) $(AFLAG) $(OUTPUT)/liblbfgs.a $(OUTPUT)/*.o
dylib :
ifeq ($(OSNAME),Windows)
	@echo "dynamic linking selected"
	@echo "Compiling"
	$(FC) $(LBFGSB) $(FFLAGS) $(OUTPUT)/lbfgsb.o
	$(FC) $(LINPACK) $(FFLAGS) $(OUTPUT)/linpack.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	$(FC) $(STRING) $(FFLAGS) $(OUTPUT)/string.o
	@echo "creating static library"
	$(FC) $(SFlAG) $(OUTPUT)/liblbfgs.dll $(OUTPUT)/*.o
endif
ifeq ($(OSNAME),Linux)
	@echo "dynamic linking selected"
	@echo "Compiling"
	@echo $(OUTPUT)
	$(FC) $(LBFGSB) $(FFLAGS) $(OUTPUT)/lbfgsb.o
	$(FC) $(LINPACK) $(FFLAGS) $(OUTPUT)/linpack.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	$(FC) $(STRING) $(FFLAGS) $(OUTPUT)/string.o
	@echo "creating static library"
	$(FC) $(SFlAG) $(OUTPUT)/liblbfgs.so $(OUTPUT)/*.o
endif
ifeq ($(OSNAME),Macos)
	@echo "dynamic linking selected"
	@echo "Compiling"
	$(FC) $(LBFGSB) $(FFLAGS) $(OUTPUT)/lbfgsb.o
	$(FC) $(LINPACK) $(FFLAGS) $(OUTPUT)/linpack.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	$(FC) $(STRING) $(FFLAGS) $(OUTPUT)/string.o
	@echo "creating static library"
	$(FC) $(SFlAG) $(OUTPUT)/liblbfgs.dylib $(OUTPUT)/*.o
endif
