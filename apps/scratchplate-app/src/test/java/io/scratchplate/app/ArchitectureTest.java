package io.scratchplate.app;

import com.tngtech.archunit.core.importer.ImportOption;
import com.tngtech.archunit.junit.AnalyzeClasses;
import com.tngtech.archunit.junit.ArchTest;
import com.tngtech.archunit.lang.ArchRule;

import static com.tngtech.archunit.base.DescribedPredicate.alwaysTrue;
import static com.tngtech.archunit.core.domain.JavaClass.Predicates.resideInAPackage;
import static com.tngtech.archunit.lang.syntax.ArchRuleDefinition.noClasses;
import static com.tngtech.archunit.library.Architectures.onionArchitecture;

/**
 * Enforces the hexagonal (ports & adapters) boundaries.
 */
@AnalyzeClasses(packages = "io.scratchplate.app", importOptions = ImportOption.DoNotIncludeTests.class)
class ArchitectureTest {

    @ArchTest
    static final ArchRule hexagonal_architecture_is_respected = onionArchitecture()
            .domainModels("..domain..")
            .applicationServices("..application..")
            .adapter("web", "..adapter.in.web..")
            .adapter("config", "..adapter.out.config..")
            .withOptionalLayers(true)
            // The infrastructure package is the composition root: it is allowed to wire every layer.
            .ignoreDependency(resideInAPackage("..infrastructure.."), alwaysTrue());

    @ArchTest
    static final ArchRule core_does_not_depend_on_infrastructure = noClasses()
            .that().resideInAnyPackage("..domain..", "..application..", "..adapter..")
            .should().dependOnClassesThat().resideInAPackage("..infrastructure..");

    @ArchTest
    static final ArchRule core_is_framework_free = noClasses()
            .that().resideInAnyPackage("..domain..", "..application..")
            .should().dependOnClassesThat().resideInAnyPackage("org.springframework..", "jakarta..");
}




